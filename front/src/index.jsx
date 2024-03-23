/* @refresh reload */
import { render } from 'solid-js/web'

import './index.css'
import Leaderboard from './Leaderboard'

const root = document.getElementById('root')

render(() => <Leaderboard />, root)
