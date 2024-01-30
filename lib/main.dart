import 'dart:async';
import 'package:fluent_ui/fluent_ui.dart';
import 'package:bramletts_games/games.dart';
import 'package:bramletts_games/src/rust/frb_generated.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const App());
}

class App extends StatelessWidget {
  const App({super.key});

  @override
  Widget build(BuildContext context) {
    return FluentApp(
        title: "Bramlett's Games",
        theme: FluentThemeData(
          brightness: Brightness.dark,
          accentColor: Colors.blue,
          visualDensity: VisualDensity.comfortable,
        ),
        home: const Home());
  }
}

class Home extends StatelessWidget {
  const Home({super.key});

  @override
  Widget build(BuildContext context) {
    return NavigationView(
      appBar: NavigationAppBar(
          title: Text("Bramlett's Games",
              style: FluentTheme.of(context).typography.title),
          leading: null,
          automaticallyImplyLeading: false),
      content: const GameList(),
    );
  }
}
