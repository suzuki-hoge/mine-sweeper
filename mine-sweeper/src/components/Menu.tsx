import './Menu.css'
import { type FC } from 'react'
import { type DensityValue } from '../type'

interface Props {
  w: number
  setW: (n: number) => void
  h: number
  setH: (n: number) => void
  density: DensityValue
  setDensity: (d: DensityValue) => void
  disabled: boolean
}

export const Menu: FC<Props> = (props) => {
  return (
    <div className="menu">
      <input
        type="number"
        value={props.w}
        min={10}
        max={50}
        step={5}
        onChange={(e) => {
          props.setW(parseInt(e.target.value))
        }}
        disabled={props.disabled}
      />

      <input
        type="number"
        value={props.h}
        min={10}
        max={50}
        step={5}
        onChange={(e) => {
          props.setH(parseInt(e.target.value))
        }}
        disabled={props.disabled}
      />

      <select
        value={props.density}
        onChange={(e) => {
          props.setDensity(e.target.value as DensityValue)
        }}
        disabled={props.disabled}
      >
        <option value="low">地雷すくなめ</option>
        <option value="middle">地雷ほどほど</option>
        <option value="high">地雷いっぱい</option>
      </select>
    </div>
  )
}
