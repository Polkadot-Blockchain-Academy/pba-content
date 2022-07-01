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

    /* insert <base href="https://www.example.com/"> in the <head/> */
    /* change this if the site is served from a subdomain */
    if (baseUrl) {
      const $base = document.createElement('base')
      $base.href = baseUrl
      document.querySelector('head').append($base)
    }

    customElements.define('widget-image', WidgetImage);
    customElements.define('widget-speaker', WidgetSpeaker);

    const $reveal =  getRevealElement()
    $reveal.setAttribute('is-loaded', true)
    console.log('Polkadot (reveal.js)');
  }
};
