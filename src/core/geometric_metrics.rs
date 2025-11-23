use crate::core::types::GeometricMetrics;
use std::collections::HashMap;
use std::sync::Arc;

/// Function signature for dynamic metric rules.
type RuleFn = Arc<dyn Fn(&mut GeometricMetrics) + Send + Sync>;

/// Engine that stores and applies dynamic metric rules.
#[derive(Default)]
pub struct GeometricMetricEngine {
    rules: HashMap<String, RuleFn>,
}

impl GeometricMetricEngine {
    /// Create a new engine instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register or replace a rule.
    pub fn register_rule<F>(&mut self, name: impl Into<String>, rule: F)
    where
        F: Fn(&mut GeometricMetrics) + Send + Sync + 'static,
    {
        self.rules.insert(name.into(), Arc::new(rule));
    }

    /// Remove an existing rule.
    pub fn remove_rule(&mut self, name: &str) -> bool {
        self.rules.remove(name).is_some()
    }

    /// Apply a single rule if it exists.
    pub fn apply_rule(&self, name: &str, metrics: &mut GeometricMetrics) -> bool {
        if let Some(rule) = self.rules.get(name) {
            rule(metrics);
            true
        } else {
            false
        }
    }

    /// Apply all registered rules.
    pub fn apply_all(&self, metrics: &mut GeometricMetrics) {
        for rule in self.rules.values() {
            rule(metrics);
        }
    }

    /// List names of all registered rules.
    pub fn rule_names(&self) -> Vec<String> {
        self.rules.keys().cloned().collect()
    }

    /// Number of registered rules.
    pub fn len(&self) -> usize {
        self.rules.len()
    }

    /// Returns true if no rules are registered.
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_and_apply_rule() {
        let mut engine = GeometricMetricEngine::new();
        engine.register_rule("boost_v", |metrics| metrics.v_geometric += 0.5);

        let mut metrics = GeometricMetrics {
            v_geometric: 1.0,
            s_geometric: 1.0,
            q_oscillator: 1.0,
            quaternion_coherence: 0.9,
            emergent_electron_mass: 9.1e-31,
            fine_structure_constant: 1.0 / 137.0,
            zitterbewegung_entropy: 0.5,
            topological_winding: 8.9,
            custom_metrics: HashMap::new(),
        };

        assert!(engine.apply_rule("boost_v", &mut metrics));
        assert_eq!(metrics.v_geometric, 1.5);
    }
}
