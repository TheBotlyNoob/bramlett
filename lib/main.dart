import 'dart:async';

import 'package:file_picker/file_picker.dart';
import 'package:fluent_ui/fluent_ui.dart';
import 'package:stroke_text/stroke_text.dart';
import 'package:bramletts_games/src/rust/api/games.dart';
import 'package:bramletts_games/src/rust/frb_generated.dart';
import 'package:url_launcher/link.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const App());
}

class App extends StatefulWidget {
  const App({super.key});

  @override
  State<StatefulWidget> createState() => _AppState();
}

class _AppState extends State<App> {
  int navIdx = 0;

  @override
  Widget build(BuildContext context) {
    return FluentApp(
        title: 'Bramlett\'s Games',
        theme: FluentThemeData(
          brightness: Brightness.dark,
          accentColor: Colors.blue,
          visualDensity: VisualDensity.comfortable,
        ),
        home: NavigationView(
          appBar: const NavigationAppBar(title: Text("Bramlett's Games")),
          pane: NavigationPane(
            selected: navIdx,
            onChanged: (newIdx) {
              setState(() {
                navIdx = newIdx;
              });
            },
            displayMode: PaneDisplayMode.auto,
            items: [
              PaneItem(
                  body: const GameList(),
                  icon: const Icon(FluentIcons.download),
                  title: const Text("Download Games")),
              PaneItem(
                  body: const Center(child: Text('b')),
                  icon: const Icon(FluentIcons.blocked),
                  title: const Text("Unblocked Browser"))
            ],
          ),
        ));
  }
}

class GameList extends StatefulWidget {
  const GameList({super.key});

  @override
  State<GameList> createState() => _GameListState();
}

class _GameListState extends State<GameList> {
  late Future<Games> futureGames;

  @override
  void initState() {
    super.initState();
    futureGames = fetchGames();
  }

  @override
  Widget build(BuildContext context) {
    return FutureBuilder<Games>(
      future: futureGames,
      builder: (context, snapshot) {
        if (snapshot.hasData) {
          return ScaffoldPage.scrollable(children: [
            Wrap(
                spacing: 10.0,
                runSpacing: 10.0,
                children: snapshot.data!.games
                    .map((game) => GameWidget(game: game))
                    .toList(growable: false))
          ]);
        } else if (snapshot.hasError) {
          return Text('${snapshot.error}');
        }

        // By default, show a loading spinner.
        return const ProgressRing();
      },
    );
  }
}

class GameWidget extends StatefulWidget {
  const GameWidget({super.key, required this.game});

  final Game game;

  @override
  State<GameWidget> createState() => _GameWidgetState();
}

class _GameWidgetState extends State<GameWidget> {
  Timer? progressPoll;
  (int, int)? progress;

  @override
  Widget build(BuildContext context) {
    final theme = FluentTheme.of(context);

    return SizedBox(
        width: 200,
        height: 300,
        child: Container(
            decoration: BoxDecoration(
                image: DecorationImage(
                    image: NetworkImage(widget.game.icon), fit: BoxFit.cover)),
            child: Card(
                child: Column(children: [
              StrokeText(
                  text: widget.game.name,
                  textStyle: theme.typography.subtitle,
                  strokeColor: theme.inactiveBackgroundColor,
                  strokeWidth: 10),
              progress == null
                  ? FilledButton(
                      child: const Text("Download"),
                      onPressed: () => showDownloadDialog(context))
                  : ProgressBar(value: (progress!.$1 / progress!.$2) * 100)
            ]))));
  }

  @override
  void dispose() {
    super.dispose();
    progressPoll?.cancel();
  }

  void showDownloadDialog(BuildContext context) async {
    await showDialog<String>(
        context: context,
        builder: (context) => ContentDialog(
                title: Text('Download ${widget.game.name}'),
                content: const Text("""
1. Click the download button below. A browser should open.

2. Click the "Click here to unlock" button until the download button appears.

CLOSE ANY NEW TABS IF THEY OPEN. These are advertisements and may contain viruses.

3. Click the "DOWNLOAD" button.

4. Once the file has downloaded, click the "Choose File" button below.
          """),
                actions: [
                  Button(
                      child: const Text("Close"),
                      onPressed: () => Navigator.pop(context)),
                  Link(
                      uri: Uri.parse(widget.game.url),
                      target: LinkTarget.blank,
                      builder: (context, followLink) => FilledButton(
                          onPressed: followLink,
                          child: const Text("Download"))),
                  FilledButton(
                      onPressed: () => filePicker(context),
                      child: const Text("Choose File"))
                ]));
  }

  void filePicker(BuildContext context) async {
    FilePickerResult? result = await FilePicker.platform.pickFiles(
        type: FileType.custom, allowedExtensions: ["7z"], withData: true);

    if (result != null) {
      PlatformFile file = result.files.first;

      var watcher = await extractZip(bytes: file.bytes!, game: widget.game);

      progressPoll = Timer.periodic(const Duration(seconds: 1), (timer) {
        setState(() {
          progress = getWatcher(obj: watcher);
        });

        // if (progress != null) {
        //   if (progress!.$1 == progress!.$2) {
        //     progressPoll?.cancel();
        //   }

        //   progress = null;
        // }
      });
    }
  }
}
