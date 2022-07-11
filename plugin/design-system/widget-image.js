export default class WidgetImage extends HTMLElement {
  static get observerdAttributes() {
    return ['src', 'fullscreen'];
  }
  get src() {
    return this.getAttribute('src') || '';
  }
  get fullscreen() {
    return this.getAttribute('fullscreen') === 'true';
  }
  get baseUrl() {
    const $base = document.querySelector('head base')
    if ($base && $base.href) {
      return $base.href
    }
  }

  connectedCallback() {
    this.render();
  }

  render() {
    if (this.src) {
      const $img = document.createElement('img');
      $img.setAttribute('loading', 'lazy')
      $img.addEventListener('load', () => {
        this.setAttribute('is-loaded', true)
      })
      if (this.baseUrl) {
        $img.src = `${this.baseUrl}/${this.src}`;
      } else {
        $img.src = this.src;
      }
      this.append($img);
    }
  }
}
