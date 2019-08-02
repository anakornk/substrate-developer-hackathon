/**
 * Sample React Native App
 * https://github.com/facebook/react-native
 *
 * @format
 * @flow
 */

import React, {Component} from 'react';
import {Platform, StyleSheet, Text, View} from 'react-native';
import { Api, WsProvider } from "@cennznet/api";

const instructions = Platform.select({
  ios: 'Press Cmd+R to reload,\n' + 'Cmd+D or shake for dev menu',
  android:
    'Double tap R on your keyboard to reload,\n' +
    'Shake or press menu button for dev menu',
});

type Props = {};
export default class App extends Component<Props> {
  constructor(props) {
    super(props);
    this.state = {
      text: '0'
    }
        // Initialise the provider to connect to the local node
    const providerUrl = Platform.select({
      ios: "ws://127.0.0.1:9944",
      android: "ws://10.0.2.2:9944"
    });
    this.provider = new WsProvider(providerUrl);
  }

  async componentDidMount() {
    this.api = await Api.create({
      provider: this.provider
    });
    this.api.rpc.chain.subscribeNewHead(header => {
      this.setState({
        text: `Chain is at #${header.blockNumber}`
      });
    });
  }
  
  render() {
    
    return (
      <View style={styles.container}>
        <Text style={styles.welcome}>Welcome to Sylo Connected App</Text>
        <Text>{this.state.text}</Text>
      </View>
    );
  }
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    backgroundColor: '#F5FCFF',
  },
  welcome: {
    fontSize: 20,
    textAlign: 'center',
    margin: 10,
  },
  instructions: {
    textAlign: 'center',
    color: '#333333',
    marginBottom: 5,
  },
});
