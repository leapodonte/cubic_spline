import * as React from 'react';
import Points from './comps/Points'
import Settings from './comps/Settings'
import {
  isNum,
  NullNum,
  PointType,
  SplineSettings,
  TENSION_DEFAULT,
  NOS_DEFAULT,
} from './glob'
import Spline from './Spline'

interface IState {
  points: PointType[]
  tension: number
  numOfSegments: number
}


const startPoints: PointType[] = [
  [0, 2],
  [123, 32],
  [150, 113],
  [200, 77],
  [319, 217],
  [419, 50],
  [509, 335],
  [650, 375],
  [700, 133],
  [870, 133],
]

class App extends React.Component<{}, IState> {

  spline: Spline
  canvEl = React.createRef<HTMLCanvasElement>()

  constructor (props: {}) {
    super(props)
    this.spline = new Spline({
      getCanvasEl: () => this.canvEl.current
    })
  }

  state: IState = {
    points: startPoints,
    tension: TENSION_DEFAULT,
    numOfSegments: NOS_DEFAULT,
  }

  getFlattenValidPoints(points: PointType[]): number[] {
    const canv = this.canvEl.current
    if (canv === null) return []
    const h = canv.height

    const validFlattenPoints: number[] = []

    points
      .forEach(([x, y]: PointType) => {
        if (isNum(x) && isNum(y)) {
          validFlattenPoints.push(x, h - y)
        }
      })

    return validFlattenPoints
  }

  handlePointsChange = (pts: Array<[NullNum, NullNum]>) => {
    this.setState({ points: pts })
  }

  handleSettingsChange = (newSettings: SplineSettings) => {
    this.setState({ ...newSettings })
  }

  componentDidUpdate() {
    this.redraw()
  }

  componentDidMount() {
    this.redraw()
  }


  redraw() {
    const pts = this.getFlattenValidPoints(this.state.points)
    this.spline.redraw(pts, this.collectSettings())
  }

  collectSettings(): SplineSettings {
    const st = this.state
    return {
      tension: st.tension,
      numOfSegments: st.numOfSegments,
    }
  }

  render() {

    return (
      <div className="App">
        <h1 className="title is-1">Demo of cubic_spline</h1>
        <div className="links">
          <a href="https://crates.io/crates/cubic_spline/"><img src="https://img.shields.io/crates/v/cubic_spline.svg" /></a>
          <a href="https://github.com/emgyrz/cubic_spline">Repository</a>
        </div>

        <div className="canvasWrp">
          <canvas ref={this.canvEl} width="900" height="400">
          </canvas>
        </div>

        <div className="controls columns section">

          <Points points={this.state.points} onChange={this.handlePointsChange} />
          <Settings settings={this.collectSettings()} onChange={this.handleSettingsChange} />

        </div>
      </div>
    );
  }

}

export default App;
