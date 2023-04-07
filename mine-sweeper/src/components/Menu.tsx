import './Menu.css'
import { type FC } from 'react'
import {
  type DensityValue,
  type DotValue,
  type SizeValue,
  type StatusValue,
} from '../type'
import { invoke } from '@tauri-apps/api/tauri'

interface Props {
  status: StatusValue
  setStatus: (s: StatusValue) => void
  size: SizeValue
  setSize: (s: SizeValue) => void
  density: DensityValue
  setDensity: (d: DensityValue) => void
  setDots: (ds: DotValue[][]) => void
}

export const Menu: FC<Props> = (props) => {
  // バックエンドインスタンスを生成する
  const newGame: () => void = () => {
    invoke('new', {})
      .then(() => {
        props.setStatus('init')
      })
      .catch(console.log)
  }

  // 盤面を生成する ( 地雷生成は初回クリック時まで行われない )
  const configureGame: () => void = () => {
    invoke<{ dots: DotValue[][] }>('configure', {
      size: props.size,
      density: props.density,
    })
      .then((response) => {
        props.setDots(response.dots)
        props.setStatus('play')
      })
      .catch(console.log)
  }

  // // for style debugging
  // useEffect(() => {
  //   newGame()
  //   configureGame()
  // }, [])

  return (
    <div className="menu">
      <div className="size">
        <p>
          <input
            id="size-1"
            type="radio"
            checked={props.size === 'small'}
            onChange={() => {
              props.setSize('small')
            }}
            disabled={props.status !== 'init'}
          />
          <label htmlFor="size-1">盤面ちいさめ</label>
        </p>
        <p>
          <input
            id="size-2"
            type="radio"
            checked={props.size === 'middle'}
            onChange={() => {
              props.setSize('middle')
            }}
            disabled={props.status !== 'init'}
          />
          <label htmlFor="size-2">盤面ほどほど</label>
        </p>
        <p>
          <input
            id="size-3"
            type="radio"
            checked={props.size === 'large'}
            onChange={() => {
              props.setSize('large')
            }}
            disabled={props.status !== 'init'}
          />
          <label htmlFor="size-3">盤面おおきい</label>
        </p>
      </div>

      <div className="density">
        <p>
          <input
            id="density-1"
            type="radio"
            checked={props.density === 'low'}
            onChange={() => {
              props.setDensity('low')
            }}
            disabled={props.status !== 'init'}
          />
          <label htmlFor="density-1">地雷すくなめ</label>
        </p>
        <p>
          <input
            id="density-2"
            type="radio"
            checked={props.density === 'middle'}
            onChange={() => {
              props.setDensity('middle')
            }}
            disabled={props.status !== 'init'}
          />
          <label htmlFor="density-2">地雷ほどほど</label>
        </p>
        <p>
          <input
            id="density-3"
            type="radio"
            checked={props.density === 'high'}
            onChange={() => {
              props.setDensity('high')
            }}
            disabled={props.status !== 'init'}
          />
          <label htmlFor="density-3">地雷たくさん</label>
        </p>
      </div>

      <div className="buttons">
        <input
          type="button"
          value="Reset"
          onClick={newGame}
          disabled={props.status === 'init'}
        />
        <input
          type="button"
          value="Start"
          onClick={configureGame}
          disabled={props.status !== 'init'}
        />
      </div>

      <div className="state">
        {props.status === 'clear'
          ? 'Clear 🎉'
          : props.status === 'bomb'
          ? 'Game Over 💥'
          : ''}
      </div>
    </div>
  )
}
