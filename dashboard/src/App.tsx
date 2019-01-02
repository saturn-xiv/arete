import { DefaultButton } from 'office-ui-fabric-react/lib/Button'
import * as React from 'react'
import { Provider } from 'react-redux'
import { createStore } from 'redux'

import { rootReducers } from './reducers'

const store = createStore(rootReducers)

class App extends React.Component {
  public render() {
    return (<Provider store={store}>
      <div>
        <DefaultButton>aaa</DefaultButton>
      </div></Provider>)
  }
}

export default App;
