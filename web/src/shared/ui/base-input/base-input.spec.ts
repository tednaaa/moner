import { flushPromises, mount, shallowMount } from '@vue/test-utils'

import BaseInput from './base-input.vue';
import { useForm } from 'vee-validate';
import { object, string } from 'zod';
import { defineComponent } from 'vue';

describe('BaseInput.vue', () => {
  it('should render input field with left and right slots', () => {
    const leftSlotText = 'left test'
    const rightSlotText = 'right test'
    const wrapper = shallowMount(BaseInput, {
      slots: { left: leftSlotText, right: rightSlotText },
      props: { name: 'test', label: 'Test', type: 'text' }
    }).text();

    expect(wrapper).contains(leftSlotText)
    expect(wrapper).contains(leftSlotText)
  })

  it.skip('should show error message if prop provided', async () => {
    const name = 'test'
    const errorMessage = 'test error message'
    const TestInput = defineComponent({
      components: { BaseInput },
      setup() {
        useForm({
          validationSchema: object({
            [name]: string().min(10),
          }),
        });
      },
      template: `<BaseInput></BaseInput>`
    })
    const wrapper = mount(TestInput, {
      props: { name, label: 'Test', type: 'text' }
    });

    const input = wrapper.find('input')
    await input.setValue('test')
    await flushPromises()

    expect(input.element.value).toBe('test')
    expect(wrapper.text()).contains(errorMessage)
  })
})
