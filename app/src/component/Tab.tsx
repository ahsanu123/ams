import { useRef, type JSX } from 'react'
import './Tab.css'

export interface ITabProps {
  activeTab: string
  handleOnActiveTabChange: (title: string) => void
  data: {
    title: string,
    component: () => JSX.Element,
  }[]
}
export default function Tab(props: ITabProps) {

  return (
    <div className='tab-container'>
      {
        props.data.map((item) => (
          <button
            className={`button-tab ${item.title === props.activeTab ? 'active' : ''}`}
            onClick={() => props.handleOnActiveTabChange(item.title)}
          >
            {item.title}
          </button>
        ))
      }
      <hr />
      {
        props.data.find((pr) => pr.title === props.activeTab)?.component()
      }
    </div>
  )
}
