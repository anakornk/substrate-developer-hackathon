import React, { Component } from "react";
import {
  Platform,
  StyleSheet,
  View,
  StatusBar,
  TouchableOpacity
} from "react-native";
import {
  createMaterialTopTabNavigator,
  createStackNavigator,
  createAppContainer,
  createDrawerNavigator
} from "react-navigation";
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

class AuctionScreen extends React.Component {
  constructor(props) {
    super(props);
    this.api = this.props.navigation.state.params.api;
    this.state = {
      blockNumber: '0'
    }
  }

  async componentDidMount() {
    this.api.rpc.chain.subscribeNewHead(header => {
      this.setState({
        blockNumber: header.blockNumber
      });
    });
  }

  render() {
    return (
      <Container>
        <Header>
          <Left>
            <Button transparent>
              <Icon
                name="arrow-back"
                onPress={() => this.props.navigation.goBack()}
              />
            </Button>
          </Left>
          <Body>
            <Title>Auction</Title>
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
          <Button light>
            <Text>Click Me!</Text>
          </Button>
          <Text>{`${this.state.blockNumber}`}</Text>
        </Content>
      </Container>
    );
  }
}

export default AuctionScreen;