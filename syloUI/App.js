/**
 * Sample React Native App
 * https://github.com/facebook/react-native
 *
 * @format
 * @flow
 */

import React, {Component} from 'react';
import {
  Platform,
  StyleSheet,
  View,
  StatusBar,
  TouchableOpacity
} from "react-native";
import { Api, WsProvider } from "@cennznet/api";
import {
  createMaterialTopTabNavigator,
  createStackNavigator,
  createAppContainer,
  createDrawerNavigator,
  StackNavigator
  
} from 'react-navigation';
import { Container, Header, Content, Button, Text, Left, Icon, Body, Title, Right } from "native-base";
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

export default createAppContainer(DrawerNavigator);
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

// const styles = StyleSheet.create({
//   container: {
//     flex: 1,
//     justifyContent: 'center',
//     alignItems: 'center',
//     backgroundColor: '#F5FCFF',
//   },
//   welcome: {
//     fontSize: 20,
//     textAlign: 'center',
//     margin: 10,
//   },
//   instructions: {
//     textAlign: 'center',
//     color: '#333333',
//     marginBottom: 5,
//   },
// });
