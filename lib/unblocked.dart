import 'package:fluent_ui/fluent_ui.dart';

class Unblocked extends StatelessWidget {
  const Unblocked({super.key});

  @override
  Widget build(BuildContext context) {
    return ScaffoldPage.scrollable(children: [
      Text(
          'Download unblocked browser- WARNING: this doesn\'t work at all. Don\'t try it'),
      FilledButton(onPressed: () => {}, child: const Text("Download"))
    ]);
  }
}
