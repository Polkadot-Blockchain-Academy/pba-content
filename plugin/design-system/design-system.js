import WidgetSpeaker from './widget-speaker.js';
import WidgetImage from './widget-image.js';

export { WidgetSpeaker, WidgetImage };

export default {
  id: 'designSystem',
  init: (options) => {
    const {
      getRevealElement,
      getConfig,
    } = options

    const {baseUrl} = getConfig()

    /* change this if the site is served from a subdomain */
    if (baseUrl) {
      WidgetImage.baseUrl = baseUrl
      WidgetSpeaker.baseUrl = baseUrl
    }
    customElements.define('widget-image', WidgetImage);
    customElements.define('widget-speaker', WidgetSpeaker);

    const $reveal =  getRevealElement()
    $reveal.setAttribute('is-loaded', true)
    console.log('Polkadot (reveal.js)');
  }
};
