import 'dart:async';
import 'package:fluent_ui/fluent_ui.dart';
import 'package:bramletts_games/src/rust/api/games.dart';

class ProgressBarWidget extends StatefulWidget {
  const ProgressBarWidget({super.key, this.color, required this.progress});

  final Color? color;
  final Progress progress;

  @override
  State<ProgressBarWidget> createState() => _ProgressBarWidgetState();
}

class _ProgressBarWidgetState extends State<ProgressBarWidget> {
  late Timer poll;

  @override
  void initState() {
    super.initState();
    poll = Timer.periodic(const Duration(seconds: 1), (timer) {
      setState(() {});
    });
  }

  @override
  void dispose() {
    super.dispose();
    poll.cancel();
  }

  @override
  Widget build(BuildContext context) {
    final value = widget.progress.isEmpty()
        ? null
        : (widget.progress.getNumerator() / widget.progress.getDenominator()) *
            100;

    return ProgressBar(activeColor: widget.color, value: value);
  }
}
