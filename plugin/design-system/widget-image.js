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

  connectedCallback() {
    this.render();
  }

  render() {
    if (this.src) {
      const $img = document.createElement('img');
      $img.src = this.src;
      this.append($img);
    }
  }
}
