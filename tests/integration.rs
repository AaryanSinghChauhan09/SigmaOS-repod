use sigmaos_repod::*;

#[test]
fn component_is_constructible_and_runnable() {
    // Arrange
    let c = sigmaos_repod::new("sigmaos-repod").expect("valid name");
    // Act
    let out = c.run().expect("run succeeds");
    // Assert
    assert!(out.contains("sigmaos-repod"));
}
