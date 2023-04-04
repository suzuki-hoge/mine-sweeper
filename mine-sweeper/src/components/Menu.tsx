import './Menu.css'
import { type FC } from 'react'
import { type DensityValue, type StatusValue } from '../type'
import { invoke } from '@tauri-apps/api/tauri'

interface Props {
  w: number
  setW: (n: number) => void
  h: number
  setH: (n: number) => void
  density: DensityValue
  setDensity: (d: DensityValue) => void
  isPlaying: boolean
  setStatus: (s: StatusValue) => void
}

export const Menu: FC<Props> = (props) => {
  const newGame: () => void = () => {
    invoke('new', {})
      .then(() => {
        props.setStatus('init')
      })
      .catch(console.log)
  }

  return (
    <div className="menu">
      <input
        type="number"
        value={props.w}
        min={5}
        max={30}
        step={5}
        onChange={(e) => {
          props.setW(parseInt(e.target.value))
        }}
        disabled={props.isPlaying}
      />

      <input
        type="number"
        value={props.h}
        min={5}
        max={30}
        step={5}
        onChange={(e) => {
          props.setH(parseInt(e.target.value))
        }}
        disabled={props.isPlaying}
      />

      <select
        value={props.density}
        onChange={(e) => {
          props.setDensity(e.target.value as DensityValue)
        }}
        disabled={props.isPlaying}
      >
        <option value="low">地雷すくなめ</option>
        <option value="middle">地雷ほどほど</option>
        <option value="high">地雷いっぱい</option>
      </select>

      <input
        type="button"
        value="Reset"
        onClick={newGame}
        disabled={!props.isPlaying}
      />
    </div>
  )
}
