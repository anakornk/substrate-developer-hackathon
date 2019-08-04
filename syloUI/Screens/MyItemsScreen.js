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

class MyItemsScreen extends React.Component {
  static navigationOptions = {
    title: "My Items"
  };

  render() {
    const { navigate } = this.props.navigation;
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
          <Text>List all my items here</Text>
        </Content>
      </Container>
    );
  }
}

export default MyItemsScreen;
