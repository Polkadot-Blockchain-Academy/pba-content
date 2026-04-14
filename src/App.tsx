import { Accordion } from './components/Accordion'

import './App.css'
import someJson from "./syllabus.json"

export const replaceText = (v: string) => v.replace(/\d+?(?=-)/g, "").replace(/-|_/g, " ")

export const App = () => {
  return (
    <>
      <header className="site-header">
        <a href="../index.html" className="logo-link">
          <img className="site-logo" src="./assets/img/0-Shared/logo/PBA_Logo_white.png" alt="PBA" />
        </a>
        <a href="../index.html" className="back-link">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="m15 18-6-6 6-6"/></svg>
          Back
        </a>
      </header>
      <div className="page-header">
        <h1 className="page-title">Protocol Builders Program</h1>
      </div>
      <div className="accordion">
        {Object.entries(someJson.syllabus || someJson).map(r => <Accordion title={replaceText(r[0])} content={r[1]} path={"syllabus/" + r[0]}/>)}
      </div>
    </>
  )
}
