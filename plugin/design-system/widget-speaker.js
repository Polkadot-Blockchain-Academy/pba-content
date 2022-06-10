export default class WidgetSpeaker extends HTMLElement {
  static get observerdAttributes() {
    return [
      'name', 'position', 'image',
      'github', 'twitter', 'telegram', 'linkedin',
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
  get github() {
    return this.getAttribute('github')
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

  socialLinks = [
    {
      name: 'github',
      url: 'https://github.com',
    },
    {
      name: 'twitter',
      url: 'https://twitter.com',
    },
    {
      name: 'telegram',
      url: 'https://t.me',
    },
    {
      name: 'linkedin',
      url: 'https://linkedin.com/in',
    }
  ]

  connectedCallback() {
		this.render()
  }

  render() {
    const $name = document.createElement('widget-speaker-name')
    $name.innerText = this.name

    const $position = document.createElement('widget-speaker-position')
    $position.innerText = this.position

    const $speakerImage = document.createElement('widget-speaker-image')
    const $img = document.createElement('img')
    $img.src = this.image
    $speakerImage.append($img)

    const $socialLinks = document.createElement('widget-speaker-social')
    this.socialLinks.forEach(socialLink => {
      const socialHandle = this[socialLink.name]
      if (!socialHandle) {
        return
      }

      const $socialLink = document.createElement('widget-speaker-social-link')

      const $title = document.createElement('widget-speaker-social-link-title')
      $title.innerText = `${socialLink.name}:`

      const $link = document.createElement('a')
      $link.href = `${socialLink.url}/${socialHandle}`
      $link.innerText = socialHandle

      $socialLink.append($title)
      $socialLink.append($link)
      $socialLinks.append($socialLink)
    })

    const $speakerBody = document.createElement('widget-speaker-body')
    this.name && $speakerBody.append($name)
    this.position && $speakerBody.append($position)
    $speakerBody.append($socialLinks)

    this.image && this.append($speakerImage)
    this.append($speakerBody)
  }
}
