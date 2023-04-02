import './Dot.css'
import React, { type FC, type MouseEventHandler } from 'react'
import { invoke } from '@tauri-apps/api'
import { type DotValue, type StatusValue, type SweptValue } from '../type'

interface Props {
  dot: DotValue
  x: number
  y: number
  setDots: (dots: DotValue[][]) => void
  setStatus: (s: StatusValue) => void
  disabled: boolean
}

export const Dot: FC<Props> = (props) => {
  const click: MouseEventHandler<HTMLDivElement> = (
    e: React.MouseEvent<HTMLDivElement, MouseEvent>
  ) => {
    // disable os context menu
    e.preventDefault()

    if (!props.disabled) {
      invoke<{ swept: SweptValue; dots: DotValue[][] }>(
        e.type === 'click' ? 'sweep' : 'flag',
        { x: props.x, y: props.y }
      )
        .then((response) => {
          props.setDots(response.dots)
          if (response.swept === 'bomb') {
            props.setStatus('bomb')
          } else if (response.swept === 'clear') {
            props.setStatus('clear')
          }
        })
        .catch(console.log)
    }
  }

  return (
    <div
      className={`dot dot-${props.dot}`}
      onClick={click}
      onContextMenu={click}
    >
      {props.dot === 'flag' ? '▶' : props.dot}
    </div>
  )
}
