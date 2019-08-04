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

class ListScreen extends React.Component {
  static navigationOptions = {
    title: "Auction"
  };
  render() {
    const { navigate } = this.props.navigation;
    return (
      <Container>
        <Header>
          <Left>
          </Left>
          <Body>
            <Title>Auction List</Title>
          </Body>
          <Right>
            <Button transparent>
              <Icon
                name="menu"
                onPress={this.props.navigation.openDrawer}
              />
            </Button>
          </Right>
        </Header>
        <Content>
          <Button onPress={() => navigate("Auction", { name: "Jane" })}>
            <Text>Click Me!</Text>
          </Button>
        </Content>
      </Container>
    );
  }
}

export default ListScreen;
