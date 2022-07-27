import React, { useState } from "react";
import View from "react-native-ui-lib/view";
import Text from "react-native-ui-lib/text";
import { Button, Colors, Keyboard } from "react-native-ui-lib";
import TextField from "react-native-ui-lib/src/incubator/TextField";

const KeyboardAwareInsetsView = Keyboard.KeyboardAwareInsetsView;

export default function LoginScreen() {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");

  return (
    <View paddingH-25 paddingV-160>
      <TextField
        text70
        value={username}
        onChange={(e: any) => setUsername(e)}
        placeholder="Username"
        floatingPlaceholder
        floatOnFocus
        marginV-15
        fieldStyle={{
          borderBottomColor: Colors.main,
          borderBottomWidth: 1,
          borderRadius: 0.5,
        }}
        floatingPlaceholderColor={Colors.main}
      />
      <TextField
        text70
        value={username}
        onChange={(e: any) => setUsername(e)}
        placeholder="Password"
        floatingPlaceholder
        floatOnFocus
        marginV-15
        fieldStyle={{
          borderBottomColor: Colors.main,
          borderBottomWidth: 1,
          borderRadius: 0.5,
        }}
        floatingPlaceholderColor={Colors.main}
        secureTextEntry
        enableErrors
        validate={["required"]}
      />

      <Button label="Login" backgroundColor={Colors.main} color={Colors.text} />
    </View>
  );
}
