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
    
    this.providerUrl = Platform.select({
      ios: "ws://localhost:9944",
      android: "ws://10.0.2.2:9944"
    });
    
  }

  render() {
    return <AppContainer screenProps={{providerUrl: this.providerUrl}}/>;
  }
}
