import 'package:flutter/material.dart';

void main() {
  runApp(const App());
}

class App extends StatelessWidget {
  const App({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return const MaterialApp(
      home: HomePage(),
    );
  }
}

class HomePage extends StatefulWidget {
  const HomePage({Key? key}) : super(key: key);

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
        appBar: AppBar(title: const Text('Text try'), primary: true),
        body: Column(children: const <Widget>[
          Padding(
              padding: EdgeInsets.all(10),
              child: TextField(
                maxLines: 1,
                keyboardType: TextInputType.emailAddress,
                decoration: InputDecoration(
                    border: OutlineInputBorder(
                      borderRadius: BorderRadius.all(Radius.circular(10.0)),
                      borderSide: BorderSide(color: Colors.black),
                    ),
                    labelText: 'Username',
                    hintText: 'Please Input name',
                    icon: Icon(Icons.email)),
              )),
          Padding(
            padding: EdgeInsets.all(10),
            child: TextField(
              textInputAction: TextInputAction.done,
              maxLines: 1,
              keyboardType: TextInputType.text,
              obscureText: true,
              decoration: InputDecoration(
                  border: OutlineInputBorder(
                    borderRadius: BorderRadius.all(Radius.circular(10.0)),
                    borderSide: BorderSide(color: Colors.black),
                  ),
                  labelText: 'Password',
                  hintText: 'Please Input psw',
                  icon: Icon(
                    Icons.lock,
                    color: Colors.orange,
                  )),
            ),
          ),
        ]));
  }
}
