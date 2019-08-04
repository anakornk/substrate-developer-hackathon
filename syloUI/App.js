/**
 * Sample React Native App
 * https://github.com/facebook/react-native
 *
 * @format
 * @flow
 */

import React, {Component} from 'react';
import { Platform } from 'react-native';
import {
  createStackNavigator,
  createAppContainer,
  createDrawerNavigator,
} from 'react-navigation';
import AuctionScreen from './Screens/AuctionScreen';
import ListScreen from './Screens/ListScreen';
import MyItemsScreen from './Screens/MyItemsScreen';

const AuctionStack = createStackNavigator({
  List: {
    screen: ListScreen,
    navigationOptions: {
      header: null
    }
  },
  Auction: {
    screen: AuctionScreen,
    navigationOptions: {
      header: null
    }
  }
});

const DrawerNavigator = createDrawerNavigator(
  {
    Auctions: AuctionStack,
    MyItems: MyItemsScreen
  },
  {
    hideStatusBar: true,
    drawerBackgroundColor: "rgba(255,255,255,.9)",
    navigationOptions: {
      header: null,
    }
  }
);

const AppContainer = createAppContainer(DrawerNavigator);
export default class App extends Component {
  constructor(props) {
    super(props);
    this.state = {
      text: '0'
    }
        // Initialise the provider to connect to the local node
    this.providerUrl = Platform.select({
      ios: "ws://localhost:9944",
      android: "ws://10.0.2.2:9944"
    });
    
    // this.providerUrl = new WsProvider(providerUrl);
  }

  render() {
    return <AppContainer screenProps={{providerUrl: this.providerUrl}}/>;
  }
}
// type Props = {};
// export default class App extends Component<Props> {
//   constructor(props) {
//     super(props);
//     this.state = {
//       text: '0'
//     }
//         // Initialise the provider to connect to the local node
//     const providerUrl = Platform.select({
//       ios: "wss://cennznet-node-0.centrality.cloud:9944",
//       android: "wss://cennznet-node-0.centrality.cloud:9944"
//     });
//     this.provider = new WsProvider(providerUrl);
//   }

//   async componentDidMount() {
//     this.api = await Api.create({
//       provider: this.provider
//     });
//     this.api.rpc.chain.subscribeNewHead(header => {
//       this.setState({
//         text: `Chain is at #${header.blockNumber}`
//       });
//     });
//   }
  
//   render() {
    
//     return (
//       <View style={styles.container}>
//         <Text style={styles.welcome}>Welcome to Sylo Connected App</Text>
//         <Text>{this.state.text}</Text>
//       </View>
//     );
//   }
// }

