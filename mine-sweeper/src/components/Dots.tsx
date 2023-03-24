import './Dots.css'
import { Dot } from './Dot'
import { type FC } from 'react'
import { type DotValue } from '../type'

interface Props {
  dots: DotValue[][]
  setDots: (ds: DotValue[][]) => void
}

export const Dots: FC<Props> = (props) => {
  return (
    <div className="dots">
      {props.dots.map((row, y) => (
        <div key={y} className="row">
          {row.map((dot, x) => (
            <Dot
              key={`${x}${y}`}
              dot={dot}
              x={x}
              y={y}
              setDots={props.setDots}
            />
          ))}
        </div>
      ))}
    </div>
  )
}
