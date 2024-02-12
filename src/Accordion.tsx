import { useState } from 'react'

interface AccordionInterface {
  title: string
  content: string
}

export const Accordion = ({ title, content }: AccordionInterface) => {
  const [isActive, setIsActive] = useState(false)

  return (
    <div className="accordion-item">
      <div className="accordion-title" onClick={() => setIsActive(!isActive)}>
        <div>{title}</div>
        <div>{isActive ? '-' : '+'}</div>
      </div>
      {isActive && <div className="accordion-content">{content}</div>}
    </div>
  )
}
