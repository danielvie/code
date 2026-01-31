import java.io.File;
import org.sikuli.script.Match;
import org.sikuli.script.Pattern;
import org.sikuli.script.Screen;

public class DragDropBot {

    public static void main(String[] args) {
        Screen screen = new Screen();

        // get template images
        String drag_path = new File("template_drag.png").getAbsolutePath();
        String drop_path = new File("template_drop.png").getAbsolutePath();

        try {
            // load patterns
            Pattern drag_item = new Pattern(drag_path).similar(0.80);
            Pattern drop_zone = new Pattern(drop_path).similar(0.80);

            // scanning boxes
            System.out.print("scanning for drag box... ");
            Match source = screen.exists(drag_item, 2);

            if (source != null) {
                System.out.println("[FOUND]");
                source.highlight(1); // blink red
            } else {
                System.err.println("[NOT FOUND]");
            }

            System.out.print("scanning for drop box...  ");
            Match target = screen.exists(drop_zone, 3); // Wait up to 3 seconds

            if (target != null) {
                System.out.println("[FOUND]");
                target.highlight(1); // blink red
            } else {
                System.err.println("[NOT FOUND]");
            }

            // drag box to position
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
        } catch (Exception e) {
            e.printStackTrace();
        }

        System.exit(0);
    }
}
