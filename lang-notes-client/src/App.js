import 'bootstrap/dist/css/bootstrap.min.css'
import 'tachyons/css/tachyons.min.css'
// import './App.css';
// import './NewWord/NewWord'
// import './WordSearch/WordSearch'
// import NewWord from './NewWord/NewWord';
import WordSearch from './WordSearch/WordSearch';
import Cluster from './Cluster/Cluster';
import Definition from './Definition/Definition';

import {
  BrowserRouter as Router,
  Switch,
  Route,
  // Link
} from "react-router-dom";

function App() {
  return (
    <div className="App">
      <Router>
        <Switch>
          <Route path="/definition/:id">
            <Definition />
          </Route>
          <Route path="/cluster/:id">
            <Cluster />
          </Route>
          <Route path="/">
            <WordSearch />
          </Route>
        </Switch>
      </Router>
    </div>
  );
}

export default App;
