import { shallowMount } from '@vue/test-utils'

import BaseButton from './base-button.vue';

describe('BaseButton.vue', () => {
  it('should render button', () => {
    const defaultSlotText = 'apple test'
    const wrapper = shallowMount(BaseButton, { slots: { default: defaultSlotText } });

    const button = wrapper.find('button')

    expect(button.text()).contains(defaultSlotText)
  })
})
