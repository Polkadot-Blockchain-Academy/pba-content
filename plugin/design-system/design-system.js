import WidgetSpeaker from './widget-speaker.js';
import WidgetImage from './widget-image.js';

export { WidgetSpeaker, WidgetImage };

export default {
  id: 'designSystem',
  init: () => {
    console.log('Reveal Polkadot design-system');
    customElements.define('widget-speaker', WidgetSpeaker);
    customElements.define('widget-image', WidgetImage);
  }
};
