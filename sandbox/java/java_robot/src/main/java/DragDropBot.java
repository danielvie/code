import java.io.File;
import org.sikuli.script.Match;
import org.sikuli.script.Pattern;
import org.sikuli.script.Screen;

public class DragDropBot {

    public static void main(String[] args) {
        Screen screen = new Screen();

        String dragPath = new File("template_drag.png").getAbsolutePath();
        String dropPath = new File("template_drop.png").getAbsolutePath();

        try {
            Match[] boxes = detectBoxes(screen, dragPath, dropPath);
            move(screen, boxes[0], boxes[1]);
        } catch (Exception e) {
            e.printStackTrace();
        }

        System.exit(0);
    }

    private static Match[] detectBoxes(
        Screen screen,
        String dragPath,
        String dropPath
    ) throws Exception {
        Pattern dragItem = new Pattern(dragPath).similar(0.80);
        Pattern dropZone = new Pattern(dropPath).similar(0.80);

        System.out.print("scanning for drag box... ");
        Match source = screen.exists(dragItem, 1);

        if (source != null) {
            System.out.println("[FOUND]");
            source.highlight(1);
        } else {
            System.err.println("[NOT FOUND]");
        }

        System.out.print("scanning for drop box...  ");
        Match target = screen.exists(dropZone, 1);

        if (target != null) {
            System.out.println("[FOUND]");
            target.highlight(1);
        } else {
            System.err.println("[NOT FOUND]");
        }

        return new Match[] { source, target };
    }

    private static void move(Screen screen, Match source, Match target)
        throws Exception {
        if (source != null && target != null) {
            screen.dragDrop(source, target);
            System.out.println("Done.");
        } else {
            System.err.println(
                "\nACTION ABORTED: One or more elements were missing."
            );
            System.err.println(
                "Tip: If the box is visible in 'debug_view.png' but says [NOT FOUND] here,"
            );
            System.err.println(
                "you MUST crop new template images from that 'debug_view.png' file."
            );
        }
    }
}
