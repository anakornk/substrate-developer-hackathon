import React, { Component } from "react";
import { Platform, StyleSheet,View } from "react-native";
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

class ListScreen extends React.Component {
  static navigationOptions = {
    title: "Auction"
  };

  constructor(props) {
    super(props);
    this.state = {
      blockNumber: "haha"
    };
    let { providerUrl } = this.props.screenProps;
    this.providerUrl = new WsProvider(providerUrl);
  }

  async componentDidMount() {
    this.api = await ApiPromise.create({
      provider: this.provider
    });
  }

  render() {
    const { navigate } = this.props.navigation;
    return (
      <Container>
        <Header>
          <Left />
          <Body>
            <Title>Auction List</Title>
          </Body>
          <Right>
            <Button transparent>
              <Icon name="menu" onPress={this.props.navigation.openDrawer} />
            </Button>
          </Right>
        </Header>
        <Content>
          <Button onPress={() => navigate("Auction", { api: this.api })}>
            <Text>Click Me!</Text>
          </Button>
        </Content>
      </Container>
    );
  }
}

export default ListScreen;
