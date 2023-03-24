import './Dot.css'
import { type FC } from 'react'
import { invoke } from '@tauri-apps/api'
import { type DotValue } from '../type'

interface Props {
  dot: DotValue
  x: number
  y: number
  setDots: (dots: DotValue[][]) => void
}

const labels: { [key in DotValue]: string } = {
  Unexplored: '',
  Flag: '▶︎',
  Zero: '',
  One: '1',
  Two: '2',
  Three: '3',
  Four: '4',
  Five: '5',
  Six: '6',
  Seven: '7',
  Eight: '8',
}

export const Dot: FC<Props> = (props) => {
  const click: () => void = () => {
    invoke<string>('foo', { x: props.x, y: props.y })
      .then((s) => {
        console.log(s)
      })
      .catch(() => {})
  }

  return (
    <div className={`dot dot-${props.dot}`} onClick={click}>
      {labels[props.dot]}
    </div>
  )
}
