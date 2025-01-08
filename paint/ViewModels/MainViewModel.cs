using Avalonia;
using Avalonia.Controls;
using Avalonia.Input;
using Avalonia.Media;
using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.Input;
using System;
using System.Collections.Generic;
using System.Threading;
using System.Threading.Tasks;

namespace paint.ViewModels;

public partial class MainViewModel : ViewModelBase
{
    private readonly Canvas _drawCanvas;
    private int _delay = 10;
    private CancellationTokenSource _ctoken;
    public RelayCommand PointerPressedCommand { get; }
    public RelayCommand PointerReleasedCommand { get; }
    public RelayCommand Undo { get; }
    public RelayCommand Redo { get; }
    
    public RelayCommand<PointerEventArgs> PointerMovedCommand { get; }
    public RelayCommand<double> WheelSizeInput { get; }
    public RelayCommand<double> WheelDensityInput { get; }

    public MainViewModel(Canvas drawCanvas)
    {
        PointerPressedCommand = new RelayCommand(OnPointerPressed);
        PointerReleasedCommand = new RelayCommand(OnPointerReleased);
        Undo = new RelayCommand(UndoAct);
        Redo = new RelayCommand(RedoAct);
        PointerMovedCommand = new RelayCommand<PointerEventArgs>(OnPointerMoved);
        WheelSizeInput = new RelayCommand<double>(OnWheelSizeInput);
        WheelDensityInput = new RelayCommand<double>(OnWheelDensityInput);
        _drawCanvas = drawCanvas;
        Task.Run(DisplayPos);
    }
    private Point _currentPoint;
    public Point CurrentPoint
    {
        get => _currentPoint;
        set => SetProperty(ref _currentPoint, value);
    }
    private int _count;
    public int Count
    {
        get => _count;
        set => SetProperty(ref _count, value);
    }
    private string _pos;
    public string Pos
    {
        get => _pos;
        set => SetProperty(ref _pos, value);
    }
    private int _steps = 0;
    public int Steps
    {
        get => _steps;
        set => SetProperty(ref _steps, value);
    }
    private bool _isUndoOn = false;
    public bool IsUndoOn
    {
        get => _isUndoOn;
        set => SetProperty(ref _isUndoOn, value);
    }
    private bool _isRedoOn = false;
    public bool IsRedoOn
    {
        get => _isRedoOn;
        set => SetProperty(ref _isRedoOn, value);
    }
    private int _size = 20;
    public int Size
    {
        get => _size;
        set {
            if(value > 0 && value < 10000) SetProperty(ref _size, value);
        }
    }
    private int _drawDistance = 5;
    public int DrawDistance
    {
        get => _drawDistance;
        set => SetProperty(ref _drawDistance, value);
    }
    private Color _bColor = new Color(255, 21, 19, 23);
    public Color BColor
    {
        get => _bColor;
        set => SetProperty(ref _bColor, value);
    }
    private void OnPointerPressed()
    {
        Steps++;
        _undoneSteps.Clear();
        IsRedoOn = false;
        IsUndoOn = true;
        _ctoken = new CancellationTokenSource();
        DrawLoop(_ctoken.Token);
    }
    private void OnPointerReleased()
    {
        _ctoken.Cancel();
        lastObject = null;
    }
    private void OnPointerMoved(PointerEventArgs e)
    {
        CurrentPoint = e.GetPosition(_drawCanvas);
    }
    private async void DisplayPos()
    {
        while (true)
        {
            Pos = "X: " + CurrentPoint.X + " Y: " + CurrentPoint.Y * -1;
            await Task.Delay(10);
        }
    }
    private async void DrawLoop(CancellationToken _ct)
    {
        while (!_ct.IsCancellationRequested)
        {
        Update(new Point(CurrentPoint.X, CurrentPoint.Y * -1));
            await Task.Delay(_delay);
        }
    }
    private Point? lastObject;
    private void Update(Point position)
    {
        double _radius = Size / 2.0;
        if (position.X == 0 && position.Y == 0) return;
        if (position.X - _radius > _drawCanvas.Width || position.Y - _radius > _drawCanvas.Height) return;
        if (lastObject is Point _l) {
            double distance = Vector.Distance(new Vector(_l.X, _l.Y), new Vector(position.X, position.Y));
            if (distance == 0)
            {
                CreateElement(position);
                lastObject = position;
            }
            if (distance < _drawDistance) return;
            int objsToSpawn = (int)(distance / _drawDistance);
            double ratio = _drawDistance / distance;
            double travelx = (position.X - _l.X) * ratio;
            double travely = (position.Y - _l.Y) * ratio;
            for (int i = 1; i <= objsToSpawn; i++)
            {
                var pos = new Point(
                    travelx * i + _l.X,
                    travely * i + _l.Y);
                double D = Vector.Distance(new Vector(_l.X, _l.Y), new Vector(pos.X, pos.Y));
                CreateElement(pos);
                lastObject = pos;
            }
        }
        else
        {
            CreateElement(position);
            lastObject = position;
        }
    }
    private Dictionary<int, List<Border>> _bordersBySteps = new Dictionary<int, List<Border>>();
    private void CreateElement(Point position) {
        var circle = new Border
        {

            Width = Size,
            Height = Size,
            CornerRadius = new CornerRadius(99999),
            Background = new SolidColorBrush(BColor)
    };
        double _radius = Size / 2.0;
        Canvas.SetLeft(circle, position.X - _radius);
        Canvas.SetTop(circle, (position.Y * -1) - _radius);

        _drawCanvas.Children.Add(circle);
        Count++;
        if(!_bordersBySteps.ContainsKey(Steps) )
        {
            _bordersBySteps[Steps] = new List<Border>();
        }
        _bordersBySteps[Steps].Add(circle);
    }
    private void UndoAct()
    {
        if (_bordersBySteps.TryGetValue(Steps, out var circles))
        {
            _undoneSteps[Steps] = circles;

            foreach (var circle in circles)
            {
                _drawCanvas.Children.Remove(circle);
                Count--;
            }
            _bordersBySteps.Remove(Steps);
        }
        Steps--;
        if (Steps == 0 ) IsUndoOn = false;
        IsRedoOn = true;
    }

    private Dictionary<int, List<Border>> _undoneSteps = new Dictionary<int, List<Border>>();
    private void RedoAct()
    {
        Steps++;
        IsUndoOn = true;

        if (_undoneSteps.TryGetValue(Steps, out var circles))
        {
            _bordersBySteps[Steps] = circles;
            foreach (var circle in circles)
            {
                _drawCanvas.Children.Add(circle);
                Count++;
            }
        }
        _undoneSteps.Remove(Steps);
        if(_undoneSteps.Count == 0 ) IsRedoOn = false;
    }
    private void OnWheelSizeInput(double scroll)
    {
        if (scroll > 0) Size++;
        else Size--;
        DrawDistance = Size / 4;

    }
    private void OnWheelDensityInput(double scroll)
    {
        if (scroll > 0) DrawDistance++;
        else DrawDistance--;
    }
}
