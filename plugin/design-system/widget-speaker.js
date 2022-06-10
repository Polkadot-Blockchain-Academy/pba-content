export default class WidgetSpeaker extends HTMLElement {
  static get observerdAttributes() {
    return [
      'name', 'position', 'image',
      'twitter', 'telegram', 'linkedin',
    ]
  }
  get name() {
    return this.getAttribute('name') || 'Speaker Name Surname'
  }
  get position() {
    return this.getAttribute('position') || 'Position/Department'
  }
  get image() {
    return this.getAttribute('image')
  }
  get twitter() {
    return this.getAttribute('twitter')
  }
  get telegram() {
    return this.getAttribute('telegram')
  }
  get linkedin() {
    return this.getAttribute('linkedin')
  }

  connectedCallback() {
		this.render()
  }

  render() {
		console.log('render')
    const $name = document.createElement('widget-speaker-name')
    $name.innerHTML = this.name

    const $position = document.createElement('widget-speaker-position')
    $position.innerHTML = this.position

    this.append($name)
    this.append($position)
  }
}
