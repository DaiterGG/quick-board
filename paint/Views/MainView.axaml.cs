using Avalonia.Controls;
using System.Security.Cryptography.X509Certificates;
using Avalonia.Input;
using paint.ViewModels;

namespace paint.Views;

public partial class MainView : UserControl
{
    public MainView()
    {
        InitializeComponent();
        DataContext = new MainViewModel(DrawCanvas);
        this.PointerMoved += Plane_PointerMoved;
        var textBoxS = this.FindControl<TextBox>("SizeTextBox");
        textBoxS.PointerWheelChanged += OnPointerWheelSizeChanged; 
        var textBoxD = this.FindControl<TextBox>("DensityTextBox");
        textBoxD.PointerWheelChanged += OnPointerWheelDensityChanged;
    }
    private void Plane_PointerPressed(object? sender, PointerPressedEventArgs e)
    {
        if (DataContext is MainViewModel viewModel)
        {
            viewModel.PointerPressedCommand.Execute(null);
        }
    }

    private void Plane_PointerReleased(object? sender, PointerReleasedEventArgs e)
    {
        if (DataContext is MainViewModel viewModel)
        {
            viewModel.PointerReleasedCommand.Execute(null);
        }
    }
    private void Plane_PointerMoved(object? sender, PointerEventArgs e)
    {
        if (DataContext is MainViewModel viewModel)
        {
            viewModel.PointerMovedCommand.Execute(e);
        }
    }
    private void OnPointerWheelSizeChanged(object? sender, PointerWheelEventArgs e)
    {
        if (DataContext is MainViewModel viewModel)
        {
            viewModel.WheelSizeInput.Execute(e.Delta.Y);
        }
    }
    private void OnPointerWheelDensityChanged(object? sender, PointerWheelEventArgs e)
    {
        if (DataContext is MainViewModel viewModel)
        {
            viewModel.WheelDensityInput.Execute(e.Delta.Y);
        }
    }
}
