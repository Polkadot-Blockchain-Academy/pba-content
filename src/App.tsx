import { Accordion } from './components/Accordion'

import './App.css'
import someJson from "./syllabus.json"
import Logo from "./assets/pba-logo-white.svg?react";

export const replaceText = (v: string) => v.replace(/\d+-*/g, "").replace(/-/g, " ").replace(/_/g, " ")

export const App = () => {
  return (
    <>
      <header className="site-header site-header--small">
        <menu className="menu menu--header">
          <li className="menu-item">
            <Logo className="site-logo" />
          </li>
          <li className="menu-item">
            <h2>PBA Agenda</h2>
          </li>
        </menu>
      </header>
      <div className="accordion">
        {
          Object.entries(someJson).map(r => <Accordion title={replaceText(r[0])} content={r[1]} />)
        }
      </div>
    </>
  )
} 