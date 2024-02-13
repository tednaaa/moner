describe('auth', () => {
  it('visits the app root url', () => {
    cy.visit('/auth/login');
    cy.get('h1').should('contain.text', 'Log in via')
  })
})
