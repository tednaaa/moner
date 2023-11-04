describe("simple spec", () => {
  it("passes", () => {
    cy.visit("/");

    expect(true).equals(true);
  });
});
