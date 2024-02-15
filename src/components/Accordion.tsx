import { useState } from 'react'
import './Accordion.css'
import { replaceText } from '../App'

interface AccordionInterface {
  title: string
  content: string | JSX.Element | object
  path: string
}

export const Accordion = ({ title, content, path }: AccordionInterface) => {
  
  const [isActive, setIsActive] = useState(false)

  return (
    <div className="accordion-item">
      <div className="accordion-title" onClick={() => setIsActive(!isActive)}>
        <div>{title}</div>
        <div>{isActive ? '-' : '+'}</div>
      </div>
      {isActive && <div className="accordion-draw">{
        Object.entries(content).map(r => {
          const text = replaceText(r[0])
          if (Object.values(r[1])[0] === undefined) {
            return (
              <a href={path + "/" + r[0] + "-slides.html"}>
                <div className="accordion-content">{text}</div>
              </a>
            )
          } else {
            return <Accordion title={text} content={r[1]} path={path + "/" + r[0]} />
          }
        })
      }</div>}
    </div>
  )
}
