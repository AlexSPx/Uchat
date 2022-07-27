import React from "react";
import View from "react-native-ui-lib/view";
import Text from "react-native-ui-lib/text";
import { Button, Colors } from "react-native-ui-lib";
import { NativeStackScreenProps } from "@react-navigation/native-stack";
import { RootStackParamList } from "../App";
import { Entypo } from "@expo/vector-icons";

type Props = NativeStackScreenProps<RootStackParamList, "Welcome">;

export default function WelcomeScreen({ navigation }: Props) {
  return (
    <View paddingH-25 paddingV-160 background flex>
      <View center>
        <Entypo name="chat" size={128} color={Colors.main} />
        <Text center main h2 style={{ fontWeight: "bold" }}>
          welcome to
        </Text>
        <Text center h1 style={{ fontWeight: "bold" }}>
          UCHAT
        </Text>
      </View>
      <View marginT-30>
        <Button
          marginV-5
          label={"Login"}
          backgroundColor={Colors.main}
          size={Button.sizes.large}
          onPress={() => navigation.navigate("Login")}
        />
        <Button
          marginV-5
          label={"Register"}
          size={Button.sizes.large}
          backgroundColor={Colors.secondary}
          onPress={() => navigation.navigate("Login")}
        />
      </View>
    </View>
  );
}
