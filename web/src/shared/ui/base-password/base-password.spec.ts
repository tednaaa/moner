import { mount } from '@vue/test-utils'

import BasePassword from './base-password.vue';

describe('BasePassword.vue', () => {
  it('should toggle password mask on eye button click', async () => {
    const wrapper = mount(BasePassword, {
      props: {
        name: 'password', label: 'Password'
      }
    });

    const toggleMaskButton = wrapper.find('[aria-label="Toggle"]')
    const input = wrapper.find('input')

    expect(input.attributes('type')).toBe('password')

    await toggleMaskButton.trigger('click');

    expect(input.attributes('type')).toBe('text')

    await toggleMaskButton.trigger('click');

    expect(input.attributes('type')).toBe('password')
  })
})
