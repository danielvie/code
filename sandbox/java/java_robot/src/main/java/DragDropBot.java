import org.sikuli.script.Location;
import org.sikuli.script.Match;
import org.sikuli.script.Screen;

public class DragDropBot {

    public static void main(String[] args) {
        Screen screen = new Screen();

        // Visible text on screen
        String sourceText = "ACTUATOR-SYS-1";
        String targetText = "Working Set Info";

        // Offsets relative to the CENTER of the matched text
        int sourceOffsetX = 0; // e.g. move left to hit file icon
        int sourceOffsetY = 0;

        int targetOffsetX = 0; // e.g. move inside drop container
        int targetOffsetY = 100;

        try {
            // Find source
            System.out.println("Searching for source text: ");
            Match source = screen.find(sourceText);
            source.highlight(2);
            printMatchPosition("SOURCE", source);

            // Find target
            System.out.println("\nSearching for target text: ");
            Match target = screen.find(targetText);
            target.highlight(2);
            printMatchPosition("TARGET", target);

            // Apply offsets
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

            // Perform drag and drop
            System.out.println("\nPerforming drag and drop...");
            screen.dragDrop(dragFrom, dropTo);
            System.out.println("Drag and drop completed.");
        } catch (Exception e) {
            System.err.println(
                "Action aborted: one or more elements not found."
            );
            e.printStackTrace();
        }

        System.exit(0);
    }

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
