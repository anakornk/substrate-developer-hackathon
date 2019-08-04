import React, { Component } from "react";
import { Platform, StyleSheet, View } from "react-native";
import {
  Container,
  Header,
  Content,
  Button,
  Text,
  Left,
  Icon,
  Body,
  Title,
  Right
} from "native-base";
import { ApiPromise, WsProvider } from "@polkadot/api";

class MyItemsScreen extends React.Component {
  static navigationOptions = {
    title: "My Items"
  };

  constructor(props) {
    super(props);
    this.state = {
      blockNumber: 'haha'
    }
    let { providerUrl, customTypes } = this.props.screenProps;
    this.provider = new WsProvider(providerUrl);
    this.customTypes = customTypes;
  }

  async componentDidMount() {
    this.api = await ApiPromise.create({
      provider: this.provider,
      types: this.customTypes
    });
    this.api.rpc.chain.subscribeNewHead(header => {
      this.setState({
        blockNumber: header.blockNumber
      });
    });
  }

  render() {
    const { navigate } = this.props.navigation;
    let { blockNumber } = this.state;
    return (
      <Container>
        <Header>
          <Left />
          <Body>
            <Title>My Items</Title>
          </Body>
          <Right>
            <Button transparent>
              <Icon name="menu" onPress={this.props.navigation.openDrawer} />
            </Button>
          </Right>
        </Header>
        <Content>
          <Text>{`Chain is at #${blockNumber}`}</Text>
          <Text>{JSON.stringify(this.props)}</Text>
        </Content>
      </Container>
    );
  }
}

export default MyItemsScreen;
