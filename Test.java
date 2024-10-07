import javafx.application.Application;
import javafx.geometry.Insets;
import javafx.scene.Scene;
import javafx.scene.control.Button;
import javafx.scene.control.Label;
import javafx.scene.control.TextArea;
import javafx.scene.control.TextField;
import javafx.scene.layout.GridPane;
import javafx.stage.Stage;

public class Test extends Application {

    @Override
    public void start(Stage primaryStage) {
        // Tworzenie layoutu (siatka)
        GridPane grid = new GridPane();
        grid.setPadding(new Insets(10, 10, 10, 10));
        grid.setVgap(8);
        grid.setHgap(10);

        // Tworzenie elementów UI
        Label label1 = new Label("Enter a number:");
        GridPane.setConstraints(label1, 0, 0);

        TextField numberInput = new TextField();
        numberInput.setPromptText("Enter a number");
        GridPane.setConstraints(numberInput, 1, 0);

        Button addButton = new Button("Add");
        GridPane.setConstraints(addButton, 0, 1);

        Button clearButton = new Button("Clear");
        GridPane.setConstraints(clearButton, 1, 1);

        TextArea resultArea = new TextArea();
        resultArea.setPromptText("Results will appear here...");
        resultArea.setPrefHeight(200);
        GridPane.setConstraints(resultArea, 0, 2, 2, 1);

        // Tworzenie akcji dla przycisków
        addButton.setOnAction(e -> {
            String input = numberInput.getText();
            try {
                int number = Integer.parseInt(input);
                resultArea.appendText("You entered: " + number + "\n");
            } catch (NumberFormatException ex) {
                resultArea.appendText("Invalid number: " + input + "\n");
            }
        });

        clearButton.setOnAction(e -> {
            numberInput.clear();
            resultArea.clear();
        });

        // Dodanie wszystkich elementów do siatki
        grid.getChildren().addAll(label1, numberInput, addButton, clearButton, resultArea);

        // Tworzenie sceny
        Scene scene = new Scene(grid, 400, 300);

        // Ustawienie i wyświetlenie sceny
        primaryStage.setScene(scene);
        primaryStage.setTitle("Text Calculator App");
        primaryStage.show();
    }

    public static void main(String[] args) {
        launch(args);
    }
}
