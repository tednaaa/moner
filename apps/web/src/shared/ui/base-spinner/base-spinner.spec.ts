import { mount } from '@vue/test-utils'

import BaseSpinner from './base-spinner.vue';

describe('BaseSpinner.vue', () => {
  it('should render with same height and width by provided size prop', async () => {
    const size = 40;
    const spinner = mount(BaseSpinner, {
      props: { size, bigPathColor: 'black', smallPathColor: 'white' },
    });

    expect(spinner.attributes('height')).toBe(size.toString());
    expect(spinner.attributes('width')).toBe(size.toString());
  })
})
