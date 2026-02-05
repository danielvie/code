import java.io.File;
import org.sikuli.script.Location;
import org.sikuli.script.Match;
import org.sikuli.script.Pattern;
import org.sikuli.script.Screen;

public class DragDropBot {

    public static void main(String[] args) {
        // runImageBasedDragDrop();
        runTextBasedDragDrop();

        System.exit(0);
    }

    // ==========================================================
    // IMAGE-BASED DRAG & DROP (Template Matching)
    // ==========================================================
    private static void runImageBasedDragDrop() {
        Screen screen = new Screen();

        String dragPath = new File("template_drag.png").getAbsolutePath();
        String dropPath = new File("template_drop.png").getAbsolutePath();

        try {
            Pattern dragItem = new Pattern(dragPath).similar(0.80);
            Pattern dropZone = new Pattern(dropPath).similar(0.80);

            System.out.print("Scanning for drag box... ");
            Match source = screen.exists(dragItem, 2);

            if (source != null) {
                System.out.println("[FOUND]");
                source.highlight(1);
            } else {
                System.err.println("[NOT FOUND]");
            }

            System.out.print("Scanning for drop box... ");
            Match target = screen.exists(dropZone, 3);

            if (target != null) {
                System.out.println("[FOUND]");
                target.highlight(1);
            } else {
                System.err.println("[NOT FOUND]");
            }

            if (source != null && target != null) {
                screen.dragDrop(source, target);
                System.out.println("Drag and drop completed.");
            } else {
                System.err.println("\nACTION ABORTED: Missing elements.");
            }
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    // TEXT-BASED DRAG & DROP (OCR)
    private static void runTextBasedDragDrop() {
        Screen screen = new Screen();

        String sourceText = "ACTUATOR-SYS-1";
        String targetText = "Working Set Info";

        int sourceOffsetX = 0;
        int sourceOffsetY = 0;

        int targetOffsetX = 0;
        int targetOffsetY = 100;

        try {
            System.out.println("Searching for source text...");
            Match source = screen.find(sourceText);
            source.highlight(2);
            printMatchPosition("SOURCE", source);

            System.out.println("\nSearching for target text...");
            Match target = screen.find(targetText);
            target.highlight(2);
            printMatchPosition("TARGET", target);

            Location dragFrom = source
                .getCenter()
                .offset(sourceOffsetX, sourceOffsetY);

            Location dropTo = target
                .getCenter()
                .offset(targetOffsetX, targetOffsetY);

            System.out.println(
                "\nDrag from: (" +
                    dragFrom.getX() +
                    ", " +
                    dragFrom.getY() +
                    ")"
            );
            System.out.println(
                "Drop to: (" + dropTo.getX() + ", " + dropTo.getY() + ")"
            );

            System.out.println("\nPerforming drag and drop...");
            screen.dragDrop(dragFrom, dropTo);
            System.out.println("Drag and drop completed.");
        } catch (Exception e) {
            System.err.println(
                "Action aborted: one or more elements not found."
            );
            e.printStackTrace();
        }
    }

    // DEBUG HELPER
    private static void printMatchPosition(String label, Match match) {
        System.out.println(label + " FOUND");
        System.out.println("X      = " + match.getX());
        System.out.println("Y      = " + match.getY());
        System.out.println("Width  = " + match.getW());
        System.out.println("Height = " + match.getH());
        System.out.println(
            "Center = (" +
                match.getCenter().getX() +
                ", " +
                match.getCenter().getY() +
                ")"
        );
    }
}
