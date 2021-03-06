use super::pattern_match::{MatchRule, PatternMatch};
use crate::grammar::*;
use crate::utils::{get_symmetric_expressions, hash, indent, unique_pats};
use crate::{parse_expression_pattern, scan};

use core::fmt;
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;

/// A mapping between two expression patterns.
#[derive(Clone, Debug)]
pub struct PatternMap {
    pub from: Rc<ExprPat>,
    pub to: Rc<ExprPat>,
}

impl fmt::Display for PatternMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.from.to_string(), self.to.to_string())
    }
}

#[derive(Debug)]
pub struct UnresolvedMapping {
    map: PatternMap,
    unresolved_pats: Vec<Rc<ExprPat>>,
}

impl fmt::Display for UnresolvedMapping {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut missing_pats = self
            .unresolved_pats
            .iter()
            .map(|p| format!(r#""{}""#, p))
            .collect::<Vec<_>>();
        missing_pats.sort();
        let missing_pats = missing_pats.join(", ");

        write!(
            f,
r#"Could not resolve pattern map
{}"{}"
Specifically, source "{}" is missing pattern(s) {} present in target "{}""#,
            indent("\n", 4),
            self.map,
            self.map.from,
            missing_pats,
            self.map.to
        )
    }
}

impl Error for UnresolvedMapping {}

impl PatternMap {
    /// Converts a string representation of a rule to a `PatternMap`.
    /// A rule's string form must be
    ///
    /// ```text
    /// "<expr> -> <expr>"
    /// ```
    ///
    /// Where `<expr>` is an expression pattern.
    pub fn from_str(rule: &str) -> Self {
        let split = rule.split(" -> ");
        let mut split = split
            .map(scan)
            .map(parse_expression_pattern)
            .map(|(expr, _)| expr);

        // Unofficially, rustc's expression evaluation order is L2R, but officially it is undefined.
        let from = split.next().unwrap();
        let to = split.next().unwrap();
        Self { from, to }
    }

    /// Bootstraps a `PatternMap` rule with a one-pass application of a rule set, which may include
    /// the rule itself.
    ///
    /// This allows the rule to match evaluated contexts that cannot be represented by a
    /// `PatternMap` created from just a string form. For example, the rule
    ///
    /// ```text
    /// "-(_a - _b) -> _b - _a"
    /// ```
    ///
    /// in its raw form may only be applied to an expression with explicit parentheses. By
    /// bootstrapping this rule with a rule that removes explicit parentheses, the rule can be
    /// applied on expressions with implicit parentheses (i.e. of the prefix form `(- (- _a _b))`).
    pub fn bootstrap(&self, bootstrapping_rules: &[Rule]) -> Self {
        let mut bootstrapped = self.clone();
        for rule in bootstrapping_rules.iter() {
            bootstrapped.from = rule.transform(bootstrapped.from);
            bootstrapped.to = rule.transform(bootstrapped.to);
        }
        bootstrapped
    }

    /// Checks a `PatternMap` is resolvable, returning an error if it is not.
    pub fn validate(&self) -> Option<UnresolvedMapping> {
        let unresolved_pats: Vec<Rc<_>> = unique_pats(&self.to)
            .difference(&unique_pats(&self.from))
            .map(|p| Rc::clone(*p))
            .collect();

        if unresolved_pats.is_empty() {
            return None;
        }

        Some(UnresolvedMapping {
            map: self.clone(),
            unresolved_pats,
        })
    }
}

pub enum Rule {
    PatternMap(PatternMap),
    Evaluate(fn(Rc<Expr>) -> Option<Rc<Expr>>),
}

impl Rule {
    pub fn from_fn(f: fn(Rc<Expr>) -> Option<Rc<Expr>>) -> Self {
        Self::Evaluate(f)
    }
}

impl Transformer<Rc<Expr>, Rc<Expr>> for Rule {
    /// Attempts to apply a rule on a target expression by
    ///
    /// 1. Applying the rule recursively on the target's subexpression to obtain a
    ///    partially-transformed target expression.
    ///
    /// 2. Pattern matching the lhs of the rule with the partially-transformed target expression.
    ///   - If pattern matching is unsuccessful, no application is done and the original expression
    ///     is returned.
    ///
    /// 3. Expanding the rhs of the rule using the results of the pattern matching.
    ///
    /// Examples:
    ///
    /// ```rust, ignore
    /// "$x + 0 -> $x".try_apply("x + 0")  // Some(x)
    /// "$x + 0 -> $x".try_apply("x + 1")  // None
    /// "$x + 0 -> $x".try_apply("x")      // None
    /// ```
    fn transform(&self, target: Rc<Expr>) -> Rc<Expr> {
        fn fill(cache: &mut HashMap<u64, Rc<Expr>>, t: Rc<Expr>, r: Rc<Expr>) -> Rc<Expr> {
            Rc::clone(cache.entry(hash(t.as_ref())).or_insert_with(|| r))
        }

        fn transform_inner(
            rule: &Rule,
            target: Rc<Expr>,
            cache: &mut HashMap<u64, Rc<Expr>>,
        ) -> Rc<Expr> {
            match target.as_ref() {
                Expr::Const(_) => Rc::clone(&target),
                Expr::Var(_) => Rc::clone(&target),
                Expr::BinaryExpr(binary_expr) => Expr::BinaryExpr(BinaryExpr {
                    op: binary_expr.op,
                    lhs: transform(rule, Rc::clone(&binary_expr.lhs), cache),
                    rhs: transform(rule, Rc::clone(&binary_expr.rhs), cache),
                })
                .into(),
                Expr::UnaryExpr(unary_expr) => Expr::UnaryExpr(UnaryExpr {
                    op: unary_expr.op,
                    rhs: transform(rule, Rc::clone(&unary_expr.rhs), cache),
                })
                .into(),
                Expr::Parend(expr) => {
                    let inner = transform(rule, Rc::clone(expr), cache);
                    Expr::Parend(inner).into()
                }
                Expr::Braced(expr) => {
                    let inner = transform(rule, Rc::clone(expr), cache);
                    Expr::Braced(inner).into()
                }
            }
        }

        fn transform(
            rule: &Rule,
            target: Rc<Expr>,
            cache: &mut HashMap<u64, Rc<Expr>>,
        ) -> Rc<Expr> {
            if let Some(result) = cache.get(&hash(target.as_ref())) {
                return Rc::clone(result);
            }

            let mut result = Rc::clone(&target);
            match rule {
                Rule::PatternMap(PatternMap { from, to }) => {
                    for target in get_symmetric_expressions(Rc::clone(&target)) {
                        // First, apply the rule recursively on the target's subexpressions.
                        let partially_transformed =
                            transform_inner(rule, Rc::clone(&target), cache);
                        if partially_transformed.complexity() < result.complexity() {
                            result = Rc::clone(&partially_transformed);
                        }

                        if let Some(transformed) = PatternMatch::match_rule(
                            Rc::clone(from),
                            Rc::clone(&partially_transformed),
                        )
                        // If the rule was matched on the expression, we have replacements for rule
                        // patterns -> target subexpressions. Apply the rule by transforming the
                        // rule's RHS with the replacements.
                        .map(|repls| repls.transform(Rc::clone(to)))
                        {
                            result = transformed;
                        }
                    }
                }
                Rule::Evaluate(f) => {
                    // First, apply the rule recursively on the target's subexpressions.
                    let partially_transformed = transform_inner(rule, Rc::clone(&target), cache);
                    result = f(Rc::clone(&partially_transformed)).unwrap_or(partially_transformed);
                }
            }

            fill(cache, target, result)
        }

        let mut cache: HashMap<u64, Rc<Expr>> = HashMap::new();
        transform(self, target, &mut cache)
    }
}

impl Transformer<Rc<ExprPat>, Rc<ExprPat>> for Rule {
    /// Bootstraps a rule with another (or possibly the same) rule.
    fn transform(&self, target: Rc<ExprPat>) -> Rc<ExprPat> {
        // First, apply the rule recursively on the target's subexpressions.
        let partially_transformed = match target.as_ref() {
            ExprPat::Const(_) | ExprPat::VarPat(_) | ExprPat::ConstPat(_) | ExprPat::AnyPat(_) => {
                Rc::clone(&target)
            }
            ExprPat::BinaryExpr(binary_expr) => ExprPat::BinaryExpr(BinaryExpr {
                op: binary_expr.op,
                lhs: self.transform(Rc::clone(&binary_expr.lhs)),
                rhs: self.transform(Rc::clone(&binary_expr.rhs)),
            })
            .into(),
            ExprPat::UnaryExpr(unary_expr) => {
                let rhs = self.transform(Rc::clone(&unary_expr.rhs));
                ExprPat::UnaryExpr(UnaryExpr {
                    op: unary_expr.op,
                    rhs,
                })
                .into()
            }
            ExprPat::Parend(expr) => {
                let inner = self.transform(Rc::clone(expr));
                ExprPat::Parend(inner).into()
            }
            ExprPat::Braced(expr) => {
                let inner = self.transform(Rc::clone(expr));
                ExprPat::Braced(inner).into()
            }
        };

        match self {
            Rule::PatternMap(PatternMap { from, to }) => {
                PatternMatch::match_rule(Rc::clone(from), Rc::clone(&partially_transformed))
                    .map(|repls| repls.transform(Rc::clone(to)))
            }
            // Only pattern map rules can be used for bootstrapping. Function rules should be exact.
            _ => unreachable!(),
        }
        .unwrap_or(partially_transformed)
    }
}

fn fn_name<T>(_: T) -> &'static str {
    let name = std::any::type_name::<T>();
    name.split("::").last().unwrap()
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::PatternMap(pm) => pm.to_string(),
                Self::Evaluate(fun) => fn_name(fun).to_string(),
            }
        )
    }
}

impl fmt::Debug for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PatternMap(pm) => write!(f, "{:?}", pm),
            Self::Evaluate(fun) => write!(f, "{}", fn_name(fun)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_error() {
        let err = PatternMap::from_str("_a + $b / #c * 3 -> 1 + $b * #e / _f")
            .validate()
            .unwrap();

        assert_eq!(
            err.to_string(),
            r##"Could not resolve pattern map
    "_a + $b / #c * 3 -> 1 + $b * #e / _f"
Specifically, source "_a + $b / #c * 3" is missing pattern(s) "#e", "_f" present in target "1 + $b * #e / _f""##
        );
    }

    #[test]
    fn validate_ok() {
        assert!(PatternMap::from_str("_a + $b / #c -> _a + $b")
            .validate()
            .is_none());
    }
}
