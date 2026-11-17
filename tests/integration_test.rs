//! Integration tests for SigmaOS-distribution-kernel-configuration.

use sigmaos_distribution_kernel_configuration_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
